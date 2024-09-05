import Screen from "../components/screen.tsx";
import Navbar from "../components/navbar.tsx";
import InternalLink from "../components/internal-link.tsx";
import {
  Button,
  Flex,
  FormControl,
  FormErrorMessage,
  FormLabel,
  Input,
  Spacer,
  Text,
} from "@chakra-ui/react";
import { FormEvent, useState } from "react";
import useAuthenticate from "../hooks/use-authenticate.ts";

const INITIAL_DATA = {
  name: "",
  password: "",
};

interface InputError {
  fieldName: "password" | "name";
  errorMessage: string;
}

export default function AuthenticateScreen() {
  const [state, setState] = useState<typeof INITIAL_DATA>(
    JSON.parse(JSON.stringify(INITIAL_DATA)),
  );
  const authMutation = useAuthenticate();
  const [errors, setErrors] = useState<InputError>(null);

  const handleSubmit = async (event: FormEvent) => {
    event.preventDefault();

    if (state.name.length === 0) {
      return setErrors({
        errorMessage: "This field is required",
        fieldName: "name",
      });
    }

    if (state.password.length === 0) {
      return setErrors({
        errorMessage: "This field is required",
        fieldName: "password",
      });
    }

    setErrors(null);
    await authMutation.mutateAsync({
      password: state.password,
      name: state.name,
    });
    setState(JSON.parse(JSON.stringify(INITIAL_DATA)));
  };

  const handleValueChange = ({
    currentTarget,
  }: FormEvent<HTMLInputElement>) => {
    const { value, name } = currentTarget;
    setState((prevState) => ({ ...prevState, [name]: value }));
  };
  return (
    <Screen>
      <Navbar />
      <Flex
        maxWidth={500}
        mx="auto"
        flexDirection="column"
        height="100%"
        justifyContent="center"
      >
        <form id="authenticate" onSubmit={handleSubmit}>
          <FormControl isInvalid={errors?.fieldName === "name"}>
            <FormLabel>Wallet name</FormLabel>
            <Input
              type="text"
              name="name"
              placeholder="e.g: main"
              value={state.name}
              onChange={handleValueChange}
            />
            <FormErrorMessage>{errors?.errorMessage}</FormErrorMessage>
          </FormControl>
          <Spacer mt={2} />
          <FormControl isInvalid={errors?.fieldName === "password"}>
            <FormLabel>Wallet password</FormLabel>
            <Input
              type="password"
              name="password"
              placeholder="******"
              value={state.password}
              onChange={handleValueChange}
            />
            <FormErrorMessage>{errors?.errorMessage}</FormErrorMessage>
          </FormControl>
          <Flex
            gap={2}
            mt={4}
            flexDirection="column"
            w="100%"
            justifyContent="space-between"
            alignItems="center"
          >
            <Button type="submit" colorScheme="blue">
              Authenticate
            </Button>
            <Text>Or</Text>
            <InternalLink to="/" variant="teal">
              Create wallet instead
            </InternalLink>
          </Flex>
        </form>
      </Flex>
    </Screen>
  );
}