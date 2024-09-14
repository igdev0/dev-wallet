import { useMutation } from "react-query";
import { invoke } from "@tauri-apps/api";
import { useParams } from "react-router-dom";
import useListAccounts, { Account } from "./use-list-accounts.ts";

type CreateAccountInput = {
  path: string;
  password: string;
};

export default function useCreateAccount() {
  const { wallet_id } = useParams();
  const listQuery = useListAccounts(false);
  return useMutation<Account, string, CreateAccountInput>(
    "create-account",
    async ({ path, password }) => {
      return await invoke("create_account", {
        walletId: wallet_id,
        path,
        password,
      });
    },
    {
      async onSuccess(data) {
        await listQuery.refetch();
      },
    },
  );
}