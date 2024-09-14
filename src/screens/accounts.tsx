import Screen from "../components/screen.tsx";
import {
  Badge,
  Button,
  Drawer,
  DrawerBody,
  DrawerContent,
  DrawerHeader,
  DrawerOverlay,
  Flex,
  FormControl,
  FormErrorMessage,
  FormLabel,
  Heading,
  IconButton,
  Input,
  Spacer,
  Table,
  Tbody,
  Td,
  Text,
  Th,
  Thead,
  Tr,
  useToast,
} from "@chakra-ui/react";
import useListAccounts, {
  AccountBlockchain,
  AccountNetwork,
} from "../hooks/use-list-accounts.ts";
import Navbar from "../components/navbar.tsx";
import { Copy01, Trash01 } from "@untitled-ui/icons-react";
import { FormEvent, PropsWithChildren, useEffect, useState } from "react";
import { isValidDerivationPath, removeRef } from "../utils/utils.ts";
import useCreateAccount from "../hooks/use-create-account.ts";

const INITIAL_ACCOUNT_DATA = {
  path: "",
  blockchain: AccountBlockchain.bitcoin,
  network: AccountNetwork.testnet,
  password: "",
};

const INITIAL_ERRORS = {
  path: null,
  password: null,
};

type ValidationError = {
  path: null | string;
  password: null | string;
};

interface CreateAccountProps extends PropsWithChildren {
  isDrawerOpen: boolean;
  onClose: () => void;
}

function CreateAccount({ isDrawerOpen, onClose }: CreateAccountProps) {
  const createAccountMutation = useCreateAccount();
  const toast = useToast();
  const [state, setState] = useState<typeof INITIAL_ACCOUNT_DATA>(
    removeRef(INITIAL_ACCOUNT_DATA),
  );
  const [errors, setErrors] = useState<ValidationError>(
    removeRef(INITIAL_ERRORS),
  );
  const [isOpen, setIsOpen] = useState(false);
  const handleClose = () => {
    setIsOpen(false);
    onClose();
  };

  const handleSubmit = async (event: FormEvent) => {
    event.preventDefault();
    if (state.password.length === 0) {
      return setErrors({ password: "This field is required", path: null });
    }

    if (!isValidDerivationPath(state.path)) {
      return setErrors({ password: null, path: "The path is invalid" });
    }
    try {
      await createAccountMutation.mutateAsync({
        path: state.path,
        password: state.password,
      });
      setState(removeRef(INITIAL_ACCOUNT_DATA));
      setErrors({ password: null, path: null });
      setIsOpen(false);
      onClose();
      toast({
        status: "success",
        title: "Account creation",
        position: "top-right",
        description: "The account was created 🎉",
      });
    } catch (error) {
      toast({
        status: "error",
        title: "Account creation error",
        description: `wasn't able to create account, details:${error.toString()}`,
      });
    }
  };

  const handleInputChange = ({
    currentTarget,
  }: FormEvent<HTMLInputElement | HTMLSelectElement>) => {
    setState((prev) => ({
      ...prev,
      [currentTarget.name]: currentTarget.value,
    }));
  };

  useEffect(() => {
    setIsOpen(isDrawerOpen);
  }, [isDrawerOpen]);

  return (
    <Drawer isOpen={isDrawerOpen} onClose={handleClose} placement="bottom">
      <DrawerOverlay />
      <DrawerContent>
        <DrawerHeader>Create a new account</DrawerHeader>
        <DrawerBody>
          <form onSubmit={handleSubmit}>
            <FormControl isInvalid={!!errors.path}>
              <FormLabel>Derivation path</FormLabel>
              <Input
                type="text"
                name="path"
                placeholder="e.g: 44'/0'/0'/0/0"
                value={state.path}
                onChange={handleInputChange}
              />
              <FormErrorMessage>{errors.path}</FormErrorMessage>
            </FormControl>
            <Spacer mt={3} />
            <FormControl isInvalid={!!errors.password}>
              <FormLabel>Current wallet password</FormLabel>
              <Input
                type="password"
                placeholder="**********"
                name="password"
                value={state.password}
                onChange={handleInputChange}
              />
              <FormErrorMessage>{!!errors.password}</FormErrorMessage>
            </FormControl>
            <Spacer mt={3} />
            <Button type="submit">Submit</Button>
          </form>
        </DrawerBody>
      </DrawerContent>
    </Drawer>
  );
}

export default function AccountsScreen() {
  const accountsRes = useListAccounts();
  const [isDrawerOpen, setIsDrawerOpen] = useState(false);
  const handleCreateClick = () => {
    setIsDrawerOpen(true);
  };

  return (
    <Screen>
      <Navbar />
      <CreateAccount
        isDrawerOpen={isDrawerOpen}
        onClose={() => setIsDrawerOpen(false)}
      />
      {accountsRes.data?.length ? (
        <>
          <Table overflowY="auto">
            <Thead>
              <Th>Address</Th>
              <Th>Blockchain</Th>
              <Th>Network</Th>
            </Thead>
            <Tbody>
              {accountsRes.data.map((item, index) => (
                <Tr key={index}>
                  <Td>{item.address}</Td>
                  <Td>
                    <Badge colorScheme="teal">{item.blockchain}</Badge>
                  </Td>
                  <Td>
                    <Badge>{item.network}</Badge>
                  </Td>
                  <Td gap={2} display="flex">
                    <IconButton aria-label="copy" icon={<Copy01 />} />
                    <IconButton aria-label="delete" icon={<Trash01 />} />
                  </Td>
                </Tr>
              ))}
            </Tbody>
          </Table>
          <Button colorScheme="teal" onClick={handleCreateClick}>
            Create new
          </Button>
        </>
      ) : (
        <Flex
          flexDirection="column"
          maxWidth={600}
          marginX="auto"
          h="100%"
          justifyContent="center"
          alignItems="center"
        >
          <Heading>No accounts found</Heading>
          <Text mb={4}>Please create one and you will find the list here.</Text>
          <Button colorScheme="teal" onClick={handleCreateClick}>
            Create
          </Button>
        </Flex>
      )}
    </Screen>
  );
}