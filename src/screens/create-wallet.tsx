import {
  Box,
  Button,
  Code,
  Flex,
  Heading,
  IconButton,
  Input,
  Spacer,
  Text,
} from "@chakra-ui/react";
import useMnemonics from "../hooks/use-mnemonics.ts";
import Loading from "../components/loading.tsx";
import Error from "../components/error.tsx";
import Screen from "../components/screen.tsx";
import { FormEventHandler, useCallback, useState } from "react";
import { Link } from "react-router-dom";
import { Copy01, RefreshCw01 } from "@untitled-ui/icons-react";
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

export default function MnemonicScreen() {
  const mnemonics = useMnemonics();

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

  const handleContinue = useCallback(() => {}, []);
  if (mnemonics.isLoading) {
    return <Loading />;
  }
  if (mnemonics.error) {
    return <Error error={mnemonics.error} />;
  }
  return (
    <Screen title="Setting up">
      <Spacer mt={2} />
      <Heading size="sm">Important note:</Heading>
      <Box bg="gray.200" _dark={{ bg: "gray.900" }} p={2}>
        <Text>
          Please save your recovery code somewhere safe, this will be used to
          generate the seed and the keys necessary to receive and spend coins.
        </Text>
      </Box>
      <Box
        py={2}
        gap={1}
        display="flex"
        flexWrap="wrap"
        justifyContent="center"
      >
        <Box>
          <Text>Your mnemonic</Text>
          <Code position="relative" p={2} _light={{ bg: "gray.200" }}>
            {mnemonics.data?.toString()}
            <Flex gap={1} justifyContent="flex-end">
              <IconButton aria-label="copy" size="sm" right={0} bottom={0}>
                <Copy01 width={20} />
              </IconButton>
              <IconButton aria-label="refresh" size="sm">
                <RefreshCw01 />
              </IconButton>
            </Flex>
          </Code>
          <Spacer mt={2} />
          <label htmlFor="password">
            Password
            <Input
              type="password"
              name="password"
              value={state.password}
              onChange={handleInputChange}
              placeholder="****"
            />
          </label>
          <Spacer mt={2} />
          <label htmlFor="confirmPassword">
            Confirm password
            <Input
              type="password"
              name="confirm_password"
              value={state.confirm_password}
              onChange={handleInputChange}
              placeholder="****"
            />
          </label>
        </Box>
      </Box>
      <Flex
        gap={2}
        justifyContent="center"
        alignSelf="flex-end"
        position="absolute"
        right={0}
        left={0}
        bottom={3}
        w="100%"
      >
        <Button colorScheme="teal" onClick={mnemonics.refetch}>
          Regenerate
        </Button>
        <Button colorScheme="blue" as={Link} to="/security">
          Continue
        </Button>
      </Flex>
    </Screen>
  );
}