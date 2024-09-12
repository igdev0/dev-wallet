import { useMutation } from "react-query";
import { invoke } from "@tauri-apps/api";
import { useParams } from "react-router-dom";
import useListAccounts, {
  Account,
  AccountBlockchain,
  AccountNetwork,
} from "./use-list-accounts.ts";

type CreateAccountInput = {
  path: string;
  network: AccountNetwork;
  blockchain: AccountBlockchain;
};

export default function useCreateAccount() {
  const { wallet_id } = useParams();
  const listQuery = useListAccounts(false);
  return useMutation<Account, string, CreateAccountInput>(
    "create-account",
    async ({ path, network, blockchain }) => {
      return await invoke("create_account", {
        walletId: wallet_id,
        path,
        blockchain,
      });
    },
    {
      async onSuccess(data) {
        await listQuery.refetch();
      },
    },
  );
}