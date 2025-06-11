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


    const burnToken = async () => {
      // const publicKeyBytes = await actor.get_ecdsa_public_key();
      // console.log("publicKeyBytes : ", publicKeyBytes.Ok);
      // const publicKeyHex = ethers.utils.hexlify(publicKeyBytes.Ok);
      // const canisterEthAddress = ethers.utils.computeAddress(publicKeyHex);
    
      // console.log("canisterEthAddre : ", canisterEthAddres);

      const canisterEthAddress = "0x260A5568d2002B8F601Fe1001BD2D93A212F087b";
      console.log("Canister ETH address:", canisterEthAddress);

      const provider = new ethers.providers.JsonRpcProvider(
        "https://eth-sepolia.g.alchemy.com/v2/qAGTv97zMDFslX0PDeLawNZw0wDToCu3"
      );
      const nonce = await provider.getTransactionCount(canisterEthAddress);
      const feeData: any = await provider.getFeeData();
      const chainId: any = (await provider.getNetwork()).chainId;

      const tokenABI = [
        "function transfer(address to,uint256 amount) returns (bool)",
      ];
      const token = new ethers.Contract(
        "0xDFdA108391A1EDa23CB0f6546e9F9386E4227994",
        tokenABI,
        provider
      );

      const burnAddress = "0x259B2BdaD6228bdC5Eb48c7A8c244f5F798113Dd";
      const amount = ethers.utils.parseUnits("1", 18);
      const txRequest = await token.populateTransaction.transfer(
        burnAddress,
        amount
      );
      console.log("Max Fee Per Gas:", feeData.maxFeePerGas.toNumber());
      console.log(
        "Max Priority Fee Per Gas:",
        feeData.maxPriorityFeePerGas.toNumber()
      );

      const gasLimit = await provider.estimateGas({
        from: canisterEthAddress,
        to: txRequest.to,
        data: txRequest.data,
        value: 0,
      });

      // const bigNum = BigNumber.from("0x6337");
      const tx = {
        type: 2,
        chainId,
        // gasPrice : bigNum,
        nonce,
        to: txRequest.to,
        data: txRequest.data,
        value: 0,
        maxFeePerGas: feeData.maxFeePerGas,
        maxPriorityFeePerGas: feeData.maxPriorityFeePerGas,
        gasLimit,
      };

      const unsignedTx = ethers.utils.serializeTransaction(tx);
      const txHash = ethers.utils.keccak256(unsignedTx);

      const ic = icblast({ ic: true });
      const backendActor = await ic("vrqyr-saaaa-aaaan-qzn4q-cai");
      const sig = await backendActor.sign_with_ecdsa({
        key_id: { name: "insecure_test_key_1", curve: { secp256k1: null } },
        derivation_path: [],
        message_hash: Array.from(ethers.utils.arrayify(txHash)),
      });
      const signature = new Uint8Array(sig.signature);
      const r = ethers.utils.hexlify(signature.slice(0, 32));
      const s = ethers.utils.hexlify(signature.slice(32, 64));
      const v = 27 + (sig.recovery_id ?? 0);
      const rawTx = ethers.utils.serializeTransaction(tx, { v, r, s });
      console.log("rawTx:", rawTx);
      const txHashRes = await actor.send_raw_transaction(rawTx);
      console.log("Broadcast success:", txHashRes);
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
        <div onClick={burnToken}> burn token </div>
      </div>
    </div>
  );
}
