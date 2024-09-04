import { useMutation } from "react-query";
import { invoke } from "@tauri-apps/api";
import { useToast } from "@chakra-ui/react";

export type VariablesType = {
  password: string;
  name: string;
};

export type AppError = {
  message: string;
};

export type CreateWalletDataType = {
  name: string;
};

export default function useCreateWallet() {
  const toast = useToast();
  const mutation = useMutation<CreateWalletDataType, string, VariablesType>(
    "create-wallet",
    {
      mutationFn: (variables) => {
        return invoke("create_wallet", {
          password: variables.password,
          name: variables.name,
        });
      },
      onSuccess: () => {
        toast({
          title: "Wallet created",
          description: "The wallet is been created",
          isClosable: true,
          status: "success",
        });
      },
      onError: (err) => {
        toast({
          title: "Failed to create wallet",
          description: `There was some issue preventing the wallet from creation. Details: "${err}"`,
          isClosable: true,
          status: "error",
        });
      },
    },
  );

  return mutation;
}