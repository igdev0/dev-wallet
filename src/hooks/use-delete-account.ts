import { useMutation } from "react-query";
import useListAccounts from "./use-list-accounts.ts";
import { invoke } from "@tauri-apps/api";

type DeleteAccountInput = {
  walletID: string;
  accountID: string;
  password: string;
};

type DeleteAccountResponse = {
  success: boolean;
};

export default function useDeleteAccount() {
  const listQuery = useListAccounts(false);
  return useMutation<DeleteAccountResponse, string, DeleteAccountInput>(
    "delete-wallet",
    async (input) => {
      return await invoke("delete_account", {
        walletId: input.walletID,
        accountId: input.accountID,
        password: input.password,
      });
    },
    {
      async onSuccess() {
        await listQuery.refetch();
      },
    },
  );
}