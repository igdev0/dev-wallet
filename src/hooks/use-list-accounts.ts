import { useQuery } from "react-query";
import { invoke } from "@tauri-apps/api";
import { useParams } from "react-router-dom";

enum AccountNetwork {
  testnet = "Testnet",
  mainnet = "Mainnet",
}

enum AccountBlockchain {
  bitcoin = "Bitcoin",
}

interface Account {
  id: string;
  address: string;
  network: AccountNetwork;
  blockchain: AccountBlockchain;
}

type QueryFNData = {
  wallet_id: string;
};

export default function useListAccounts() {
  let { wallet_id } = useParams<{ wallet_id: string }>();
  return useQuery<QueryFNData, String, Account[]>("list-accounts", async () => {
    if (!wallet_id) {
      throw new Error("The wallet ID is not present in the URL, aborting ...");
    }
    return await invoke("list_accounts", { walletId: wallet_id });
  });
}