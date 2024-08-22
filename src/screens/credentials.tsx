import Screen from "../components/screen.tsx";
import { Box, Button, Flex, Heading, Input } from "@chakra-ui/react";
import { FormEventHandler, useCallback, useState } from "react";
import { invoke } from "@tauri-apps/api";

const INITIAL_STATE = {
  confirm_password: "",
  password: "",
};

interface State {
  confirm_password: string;
  password: string;
}

interface InputError {
  fieldName: "confirm_password" | "password";
  errorMessage: string;
}

export default function CredentialsScreen() {
  const [error, setError] = useState<InputError>(null);
  const [state, setState] = useState<State>(
    JSON.parse(JSON.stringify(INITIAL_STATE)),
  );
  const handleSubmit = useCallback<FormEventHandler>(
    async (event) => {
      event.preventDefault();
      if (state.password !== state.confirm_password) {
        return setError({
          errorMessage: "The passwords are not matching",
          fieldName: "password",
        });
      }

      if (state.password.length === 0) {
        return setError({
          errorMessage: "The password is required",
          fieldName: "password",
        });
      }

      try {
        const res = await invoke("create_wallet", { input: state.password });
        setState(JSON.parse(JSON.stringify(INITIAL_STATE)));
      } catch (err) {
        // handle errors
      }
    },
    [state],
  );
  const handleInputChange = useCallback<FormEventHandler<HTMLInputElement>>(
    (e) => {
      const { currentTarget } = e;
      setState((prev) => ({
        ...prev,
        [currentTarget.name]: currentTarget.value,
      }));
    },
    [setState],
  );

  return (
    <Screen title="Setup password">
      <Box maxW={500} mx="auto">
        <Heading textAlign="center" mb={1} mt={4}>
          Setting up your password
        </Heading>
        <form onSubmit={handleSubmit}>
          <label htmlFor="password">
            Password
            <Input
              type="password"
              name="password"
              value={state.password}
              placeholder="*****"
              onChange={handleInputChange}
            />
            {error && error.errorMessage}
          </label>
          <label htmlFor="confim_password">
            Confirm password
            <Input
              type="password"
              value={state.confirm_password}
              name="confirm_password"
              placeholder="*****"
              onChange={handleInputChange}
            />
          </label>
          <Flex justifyContent="flex-end">
            <Button type="submit" mt={2} colorScheme="blue">
              Continue
            </Button>
          </Flex>
        </form>
      </Box>
    </Screen>
  );
}