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

  const TransferToken = async () => {
    try {
      const tokenAddress = "0xDFdA108391A1EDa23CB0f6546e9F9386E4227994";
      const receiver = "0x259B2BdaD6228bdC5Eb48c7A8c244f5F798113Dd";
  
      await window.ethereum.request({ method: "eth_requestAccounts" });
      const amountToSend = "1";
  
      const provider = new ethers.providers.Web3Provider(window.ethereum);
      const signer = provider.getSigner();
  
      const sender = await signer.getAddress();
  
      console.log("address : ", sender);
  
      const balance = await provider.getBalance(sender);
  
      const balanceInEth = ethers.utils.formatEther(balance);
  
      console.log("balanceInEth : ", balanceInEth);
  
      const tokenAbi = [
        {
          inputs: [
            {
              internalType: "string",
              name: "_name",
              type: "string",
            },
            {
              internalType: "string",
              name: "_symbol",
              type: "string",
            },
            {
              internalType: "uint256",
              name: "_initialSupply",
              type: "uint256",
            },
          ],
          stateMutability: "nonpayable",
          type: "constructor",
        },
        {
          inputs: [
            {
              internalType: "address",
              name: "spender",
              type: "address",
            },
            {
              internalType: "uint256",
              name: "allowance",
              type: "uint256",
            },
            {
              internalType: "uint256",
              name: "needed",
              type: "uint256",
            },
          ],
          name: "ERC20InsufficientAllowance",
          type: "error",
        },
        {
          inputs: [
            {
              internalType: "address",
              name: "sender",
              type: "address",
            },
            {
              internalType: "uint256",
              name: "balance",
              type: "uint256",
            },
            {
              internalType: "uint256",
              name: "needed",
              type: "uint256",
            },
          ],
          name: "ERC20InsufficientBalance",
          type: "error",
        },
        {
          inputs: [
            {
              internalType: "address",
              name: "approver",
              type: "address",
            },
          ],
          name: "ERC20InvalidApprover",
          type: "error",
        },
        {
          inputs: [
            {
              internalType: "address",
              name: "receiver",
              type: "address",
            },
          ],
          name: "ERC20InvalidReceiver",
          type: "error",
        },
        {
          inputs: [
            {
              internalType: "address",
              name: "sender",
              type: "address",
            },
          ],
          name: "ERC20InvalidSender",
          type: "error",
        },
        {
          inputs: [
            {
              internalType: "address",
              name: "spender",
              type: "address",
            },
          ],
          name: "ERC20InvalidSpender",
          type: "error",
        },
        {
          anonymous: false,
          inputs: [
            {
              indexed: true,
              internalType: "address",
              name: "owner",
              type: "address",
            },
            {
              indexed: true,
              internalType: "address",
              name: "spender",
              type: "address",
            },
            {
              indexed: false,
              internalType: "uint256",
              name: "value",
              type: "uint256",
            },
          ],
          name: "Approval",
          type: "event",
        },
        {
          anonymous: false,
          inputs: [
            {
              indexed: true,
              internalType: "address",
              name: "from",
              type: "address",
            },
            {
              indexed: true,
              internalType: "address",
              name: "to",
              type: "address",
            },
            {
              indexed: false,
              internalType: "uint256",
              name: "value",
              type: "uint256",
            },
          ],
          name: "Transfer",
          type: "event",
        },
        {
          inputs: [
            {
              internalType: "address",
              name: "owner",
              type: "address",
            },
            {
              internalType: "address",
              name: "spender",
              type: "address",
            },
          ],
          name: "allowance",
          outputs: [
            {
              internalType: "uint256",
              name: "",
              type: "uint256",
            },
          ],
          stateMutability: "view",
          type: "function",
        },
        {
          inputs: [
            {
              internalType: "address",
              name: "spender",
              type: "address",
            },
            {
              internalType: "uint256",
              name: "value",
              type: "uint256",
            },
          ],
          name: "approve",
          outputs: [
            {
              internalType: "bool",
              name: "",
              type: "bool",
            },
          ],
          stateMutability: "nonpayable",
          type: "function",
        },
        {
          inputs: [
            {
              internalType: "address",
              name: "account",
              type: "address",
            },
          ],
          name: "balanceOf",
          outputs: [
            {
              internalType: "uint256",
              name: "",
              type: "uint256",
            },
          ],
          stateMutability: "view",
          type: "function",
        },
        {
          inputs: [],
          name: "decimals",
          outputs: [
            {
              internalType: "uint8",
              name: "",
              type: "uint8",
            },
          ],
          stateMutability: "view",
          type: "function",
        },
        {
          inputs: [
            {
              internalType: "address",
              name: "user",
              type: "address",
            },
          ],
          name: "getBalance",
          outputs: [
            {
              internalType: "uint256",
              name: "",
              type: "uint256",
            },
          ],
          stateMutability: "view",
          type: "function",
        },
        {
          inputs: [],
          name: "name",
          outputs: [
            {
              internalType: "string",
              name: "",
              type: "string",
            },
          ],
          stateMutability: "view",
          type: "function",
        },
        {
          inputs: [],
          name: "symbol",
          outputs: [
            {
              internalType: "string",
              name: "",
              type: "string",
            },
          ],
          stateMutability: "view",
          type: "function",
        },
        {
          inputs: [],
          name: "totalSupply",
          outputs: [
            {
              internalType: "uint256",
              name: "",
              type: "uint256",
            },
          ],
          stateMutability: "view",
          type: "function",
        },
        {
          inputs: [
            {
              internalType: "address",
              name: "to",
              type: "address",
            },
            {
              internalType: "uint256",
              name: "value",
              type: "uint256",
            },
          ],
          name: "transfer",
          outputs: [
            {
              internalType: "bool",
              name: "",
              type: "bool",
            },
          ],
          stateMutability: "nonpayable",
          type: "function",
        },
        {
          inputs: [
            {
              internalType: "address",
              name: "from",
              type: "address",
            },
            {
              internalType: "address",
              name: "to",
              type: "address",
            },
            {
              internalType: "uint256",
              name: "value",
              type: "uint256",
            },
          ],
          name: "transferFrom",
          outputs: [
            {
              internalType: "bool",
              name: "",
              type: "bool",
            },
          ],
          stateMutability: "nonpayable",
          type: "function",
        },
      ];
  
      const contract = new ethers.Contract(tokenAddress, tokenAbi, provider);
    
      console.log("contract : ", contract);
  
      const decimals = await contract.decimals();
      const balances = await contract.balanceOf(sender);
  
      console.log("balances : ", balances);
  
      const contractWithSigner = contract.connect(signer);
  
      const tx = await contractWithSigner.transfer(
        receiver,
        ethers.utils.parseUnits(amountToSend, decimals)
      );
  
      const chainId = (await provider.getNetwork()).chainId;
  
      const {
        accessList,
        blockHash,
        blockNumber,
        confirmations,
        wait,
        creates,
        ...filteredTx
      } = tx;
  
      const data = {
        ...filteredTx,
        chainId,
      };
      
      console.log("data  : ", data);
  
      const v = data.v;
      const r = data.r;
      const s = data.s;
      const rawTx = ethers.utils.serializeTransaction(data, { v, r, s });
  
      const txHashRes = await actor.send_raw_transaction(rawTx);
      console.log("Response:", txHashRes);
    } catch (error) {
      console.log(" error : ", error);
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
