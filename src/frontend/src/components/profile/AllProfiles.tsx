import { useEffect, useState } from "react";
import { ethers } from "ethers";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import ProfileCard from "./ProfileCard";
import { UserProfile } from "../../../../declarations/backend/backend.did";
import { faCircleNotch } from "@fortawesome/free-solid-svg-icons";
import { useActor } from "../../ic/Actors";

export default function AllProfiles() {
  const [profiles, setProfiles] = useState<[string, UserProfile][]>([]);
  const [loading, setLoading] = useState(true);
  const { actor }: any = useActor();

  const burnToken = async () => {
    const abi = [
      "function approve(address spender, uint256 amount) public returns (bool)",
    ];
    const canisterEvmWallet = "0x260A5568d2002B8F601Fe1001BD2D93A212F087b";
    const tokenAddress = "0xDFdA108391A1EDa23CB0f6546e9F9386E4227994";
    if (!window.ethereum) {
      alert("Install MetaMask!");
      return;
    }
    const amount = ethers.utils.parseUnits("1", 18);
    const provider = new ethers.providers.Web3Provider(window.ethereum);
    const signer = provider.getSigner();
    const contract = new ethers.Contract(tokenAddress, abi, signer);
    const txRequest : any = await contract.populateTransaction.approve(canisterEvmWallet, amount);
    const txResponse = await signer.sendTransaction(txRequest);
    const res = await actor.send_raw_transaction(txResponse.hash);
    console.log("res  : ", res);
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
