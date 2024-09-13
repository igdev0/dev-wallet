import { useMutation, useQueryClient } from "react-query";
import { invoke } from "@tauri-apps/api";

type DeleteAccountInput = {
  walletID: string;
  password: string;
};

type DeleteAccountResponse = {
  success: boolean;
};

export default function useDeleteWallet() {
  const queryClient = useQueryClient();
  return useMutation<DeleteAccountResponse, string, DeleteAccountInput>(
    "delete-wallet",
    async (input) => {
      return await invoke("remove_wallet", {
        walletId: input.walletID,
        accountId: input.password,
      });
    },
    {
      async onSuccess() {
        await queryClient.invalidateQueries("list-accounts");
      },
    },
  );
}