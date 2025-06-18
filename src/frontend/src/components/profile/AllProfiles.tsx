import { useEffect, useState } from "react";
import { ethers } from "ethers";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import ProfileCard from "./ProfileCard";
import { UserProfile } from "../../../../declarations/backend/backend.did";
import { faCircleNotch } from "@fortawesome/free-solid-svg-icons";
import { useActor } from "../../ic/Actors";
import icblast from "@infu/icblast";

export default function AllProfiles() {
  const [profiles, setProfiles] = useState<[string, UserProfile][]>([]);
  const [loading, setLoading] = useState(true);
  const { actor }: any = useActor();

  const TransferToken = async () => {
    try {
      const sendAddress = "0x259B2BdaD6228bdC5Eb48c7A8c244f5F798113Dd";
      const tokenAddress = "0xDFdA108391A1EDa23CB0f6546e9F9386E4227994";
      const amount = ethers.utils.parseUnits("1", 18);
      const browserProvider = new ethers.providers.Web3Provider(
        window.ethereum
      );
      await browserProvider.send("eth_requestAccounts", []);
      const signer = browserProvider.getSigner();

      const tokenABI = [
        "function approve(address spender, uint256 amount) public returns (bool)",
        "function transferFrom(address from, address to, uint256 amount) public returns (bool)",
      ];

      const tokenWithSigner = new ethers.Contract(
        tokenAddress,
        tokenABI,
        signer
      );
      const approveTx = await tokenWithSigner.approve(sendAddress, amount);
      await approveTx.wait();

      const provider = new ethers.providers.JsonRpcProvider(
        "https://eth-sepolia.g.alchemy.com/v2/NP2an-FMSHKAB1qV7U0vBZP6w4g5Yiir"
      );
      const nonce = await provider.getTransactionCount(
        await signer.getAddress()
      );
      const feeData: any = await provider.getFeeData();
      const chainId = (await provider.getNetwork()).chainId;

      const txRequest = await tokenWithSigner.populateTransaction.transferFrom(
        await signer.getAddress(),
        sendAddress,
        amount
      );

      const gasLimit = await provider.estimateGas({
        from: await signer.getAddress(),
        to: tokenAddress,
        data: txRequest.data,
        value: 0,
        nonce,
        maxFeePerGas: feeData.maxFeePerGas,
        maxPriorityFeePerGas: feeData.maxPriorityFeePerGas,
        chainId,
      });

      const tx = {
        type: 2,
        chainId,
        nonce,
        from: await signer.getAddress(),
        to: tokenAddress,
        data: txRequest.data,
        value: 0,
        maxFeePerGas: feeData.maxFeePerGas,
        maxPriorityFeePerGas: feeData.maxPriorityFeePerGas,
        gasLimit: gasLimit,
        accessList: [],
      };

      console.log("Gas Limit: ", gasLimit.toString());

      const ic = icblast({ ic: true });
      const EcdsaPublicKeyActor = await ic("vrqyr-saaaa-aaaan-qzn4q-cai");

      const sig = await EcdsaPublicKeyActor.sign_with_ecdsa({
        key_id: { name: "insecure_test_key_1", curve: { secp256k1: null } },
        derivation_path: [],
        message_hash: Array.from(
          ethers.utils.arrayify(
            ethers.utils.keccak256(ethers.utils.serializeTransaction(tx))
          )
        ),
      });

      const signature = new Uint8Array(sig.signature);
      const r = ethers.utils.hexlify(signature.slice(0, 32));
      const s = ethers.utils.hexlify(signature.slice(32, 64));
      const v = 27 + (sig.recovery_id ?? 0);
      const rawTx = ethers.utils.serializeTransaction(tx, { v, r, s });

      const txHashRes = await actor.send_raw_transaction(rawTx);
      console.log("Response:", txHashRes);
    } catch (error) {
      console.log("Error:", error);
    }
  };

  useEffect(() => {
    if (!actor) return;
    (async () => {
      const response = await actor.list_profiles();
      if (response && "Ok" in response) {
        setProfiles(response.Ok);
      }
      setLoading(false);
    })();
  }, [actor]);

  return (
    <div className="w-full max-w-2xl border-zinc-700/50 border-[1px] bg-zinc-900 px-5 py-5 drop-shadow-xl rounded-3xl flex flex-col items-center">
      <div className="flex flex-col items-center w-full gap-10 py-8 md:px-8">
        <div className="text-2xl font-bold">User Profiles</div>
        <div className="flex flex-col items-center gap-5">
          {loading && (
            <div className="flex flex-col items-center justify-center w-full h-64">
              <FontAwesomeIcon className="w-4 h-4" icon={faCircleNotch} spin />
            </div>
          )}
          <div className="grid grid-cols-1 gap-5 md:grid-cols-3">
            {profiles.map((p) => (
              <ProfileCard key={p[0]} principal={p[0]} profile={p[1]} />
            ))}
          </div>
        </div>
        <button
          onClick={TransferToken}
          className="rounded-lg px-4 py-2 border-2 border-yellow-500 text-yellow-500 hover:bg-yellow-500 hover:text-yellow-100 duration-300"
        >
          {" "}
          burn token{" "}
        </button>
      </div>
    </div>
  );
}
