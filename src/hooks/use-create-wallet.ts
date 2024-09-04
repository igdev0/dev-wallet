import { useMutation } from "react-query";
import { invoke } from "@tauri-apps/api";

type DataType = {
  password: string;
  name: string;
};
export default function useCreateWallet() {
  const mutation = useMutation<_, _, DataType>("create-wallet", {
    mutationFn: (variables) => {
      return invoke("create_wallet", {
        password: variables.password,
        name: variables.name,
      });
    },
  });

  return mutation;
}