import { useQuery } from "react-query";
import { invoke } from "@tauri-apps/api";
import { useParams } from "react-router-dom";

export enum AccountNetwork {
  testnet = "Testnet",
  mainnet = "Mainnet",
}

export enum AccountBlockchain {
  bitcoin = "Bitcoin",
}

export interface Account {
  id: string;
  address: string;
  network: AccountNetwork;
  blockchain: AccountBlockchain;
}

type QueryFNData = {
  wallet_id: string;
};

export default function useListAccounts(enabled = true) {
  let { wallet_id } = useParams<{ wallet_id: string }>();
  return useQuery<QueryFNData, String, Account[]>(
    "list-accounts",
    async () => {
      if (!wallet_id) {
        throw new Error(
          "The wallet ID is not present in the URL, aborting ...",
        );
      }
      return await invoke("list_accounts", { walletId: wallet_id });
    },
    { enabled, suspense: true, useErrorBoundary: true, retry: false },
  );
}