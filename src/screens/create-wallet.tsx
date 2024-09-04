import {
  Box,
  Button,
  Code,
  Flex,
  FormControl,
  FormErrorMessage,
  FormLabel,
  Heading,
  Icon,
  IconButton,
  Input,
  Spacer,
  Text,
  Tooltip,
  useClipboard,
} from "@chakra-ui/react";
import useMnemonics from "../hooks/use-mnemonics.ts";
import Loading from "../components/loading.tsx";
import Error from "../components/error.tsx";
import Screen from "../components/screen.tsx";
import { FormEventHandler, useCallback, useState } from "react";
import { Check, Copy01, RefreshCw01 } from "@untitled-ui/icons-react";
import useCreateWallet from "../hooks/use-create-wallet.ts";
import Navbar from "../components/navbar.tsx";

const INITIAL_STATE = {
  name: "",
  confirm_password: "",
  password: "",
};

interface State {
  name: string;
  confirm_password: string;
  password: string;
}

interface InputError {
  fieldName: "confirm_password" | "password" | "name";
  errorMessage: string;
}

export default function MnemonicScreen() {
  const mnemonics = useMnemonics();
  const createWalletMut = useCreateWallet();
  const { onCopy, setValue } = useClipboard("");
  const [isCopyTooltipOpen, setIsCopyTooltipOpen] = useState(false);
  const [isRefreshTooltipOpen, setIsRefreshTooltipOpen] = useState(false);
  const [error, setError] = useState<InputError>(null);
  const [state, setState] = useState<State>(
    JSON.parse(JSON.stringify(INITIAL_STATE)),
  );
  const handleSubmit = async (event) => {
    event.preventDefault();
    if (state.name.length === 0) {
      return setError({
        errorMessage: "This field is required",
        fieldName: "name",
      });
    }
    if (state.password !== state.confirm_password) {
      return setError({
        errorMessage: "The passwords are not matching",
        fieldName: "confirm_password",
      });
    }

    if (state.password.length === 0) {
      return setError({
        errorMessage: "The password is required",
        fieldName: "password",
      });
    }
    setError(null);

    try {
      await createWalletMut.mutateAsync({
        password: state.password,
        name: state.name,
      });

      setState(JSON.parse(JSON.stringify(INITIAL_STATE)));
    } catch (_) {}
  };
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

  const handleMnemonicsCopy = () => {
    setValue(mnemonics.toString);
    onCopy();
    setIsCopyTooltipOpen(true);
    setTimeout(() => {
      setIsCopyTooltipOpen(false);
    }, 1500);
  };

  const handleRefreshMnemonics = useCallback(async () => {
    await mnemonics.refetch();
    setIsRefreshTooltipOpen(true);
    setTimeout(() => {
      setIsRefreshTooltipOpen(false);
    }, 1500);
  }, [mnemonics]);

  if (mnemonics.isLoading) {
    return <Loading />;
  }
  if (mnemonics.error) {
    return <Error error={mnemonics.error} />;
  }
  return (
    <Screen>
      <Navbar text="Setting up" />
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
        <form id="create-wallet" onSubmit={handleSubmit}>
          <Box>
            <Flex gap={1} w="100%" justifyContent="space-between" mb={2}>
              <Text>Your mnemonic</Text>
              <Flex gap={2}>
                <Tooltip
                  label="Copied!"
                  hasArrow
                  isOpen={isCopyTooltipOpen}
                  alignSelf="flex-end"
                >
                  <IconButton
                    aria-label="copy"
                    size="sm"
                    right={0}
                    bottom={0}
                    onClick={handleMnemonicsCopy}
                  >
                    <Copy01 width={20} />
                  </IconButton>
                </Tooltip>
                <Tooltip
                  label="Refreshed!"
                  hasArrow
                  isOpen={isRefreshTooltipOpen}
                >
                  <IconButton
                    aria-label="refresh"
                    size="sm"
                    onClick={handleRefreshMnemonics}
                  >
                    <RefreshCw01 />
                  </IconButton>
                </Tooltip>
              </Flex>
            </Flex>
            <Code position="relative" p={2} _light={{ bg: "gray.200" }}>
              {mnemonics.data?.toString()}
            </Code>
            <Spacer mt={2} />
            <FormControl isInvalid={error?.fieldName === "name"} as="fieldset">
              <FormLabel htmlFor="name">Wallet name</FormLabel>
              <Input
                type="text"
                name="name"
                value={state.name}
                onChange={handleInputChange}
                placeholder="E.g: Main"
              />
              <FormErrorMessage>{error?.errorMessage}</FormErrorMessage>
            </FormControl>
            <Spacer mt={2} />
            <FormControl
              isInvalid={error?.fieldName === "password"}
              as="fieldset"
            >
              <FormLabel htmlFor="password">Password</FormLabel>
              <Input
                type="password"
                name="password"
                value={state.password}
                onChange={handleInputChange}
                placeholder="****"
              />
              <FormErrorMessage>{error?.errorMessage}</FormErrorMessage>
            </FormControl>
            <FormControl isInvalid={error?.fieldName === "confirm_password"}>
              <FormLabel>Confirm password</FormLabel>
              <Input
                type="password"
                name="confirm_password"
                value={state.confirm_password}
                onChange={handleInputChange}
                placeholder="****"
              />
              <FormErrorMessage>{error?.errorMessage}</FormErrorMessage>
            </FormControl>
            <Spacer mt={2} />
          </Box>
        </form>
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
        <Button
          colorScheme="blue"
          type="submit"
          justifyItems="center"
          alignItems="center"
          form="#create-wallet"
          onClick={handleSubmit}
        >
          Submit
          <Icon ml={2}>
            <Check />
          </Icon>
        </Button>
      </Flex>
    </Screen>
  );
}