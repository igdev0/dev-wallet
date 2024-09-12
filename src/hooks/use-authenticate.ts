import { useMutation } from "react-query";
import { invoke } from "@tauri-apps/api";
import { useAppState } from "../state.ts";
import { useNavigate } from "react-router-dom";
import { useToast } from "@chakra-ui/react";

export type AuthenticateInput = {
  name: string;
  password: string;
};

export type AuthenticateData = {
  id: string;
  name: string;
};

export default function useAuthenticate() {
  const navigate = useNavigate();
  const toast = useToast();
  const mutation = useMutation<AuthenticateData, string, AuthenticateInput>(
    "authenticate",
    {
      mutationFn: async ({ password, name }) => {
        return invoke("authenticate", { name, password });
      },
      onSuccess: (data) => {
        useAppState.getState().setAuthenticated(data.name);
        navigate(`/accounts/${data.id}`);
      },
      onError: (error, variables, context) => {
        toast({
          title: "Authentication failed",
          status: "error",
          isClosable: true,
          description: `Details: ${error}`,
        });
      },
    },
  );

  return mutation;
}