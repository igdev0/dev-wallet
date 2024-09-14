import {
  Drawer,
  DrawerBody,
  DrawerContent,
  DrawerHeader,
  DrawerOverlay,
  FormControl,
  FormErrorMessage,
  FormLabel,
  Heading,
  Input,
} from "@chakra-ui/react";
import { FormEvent, PropsWithChildren, useEffect, useState } from "react";

interface ConfirmPasswordProps extends PropsWithChildren {
  isDrawerOpen: boolean;
  onConfirmation: (password: string) => Promise<any>;
}

export default function ConfirmPassword(props: ConfirmPasswordProps) {
  const [isDrawerOpen, set] = useState(false);
  const [password, setPassword] = useState("");
  const [isInvalid, setIsInvalid] = useState(false);
  const [errorMessage, setErrorMessage] = useState("");
  useEffect(() => {
    set(props.isDrawerOpen);
  }, [props.isDrawerOpen]);

  const handleDrawerClose = () => {
    set(false);
  };

  const handleSubmit = async () => {
    try {
      await props.onConfirmation(password);
    } catch (err) {
      setIsInvalid(true);
      setErrorMessage(err.toString);
    }
  };

  const handleInputChange = ({
    currentTarget,
  }: FormEvent<HTMLInputElement>) => {
    setPassword(currentTarget.value);
  };

  return (
    <Drawer
      placement="bottom"
      onClose={handleDrawerClose}
      isOpen={isDrawerOpen}
    >
      <DrawerOverlay />
      <DrawerContent>
        <DrawerHeader borderBottomWidth="1px">Basic Drawer</DrawerHeader>
        <DrawerBody>
          <Heading>Please enter your password</Heading>
          <form onSubmit={handleSubmit}>
            <FormControl isInvalid={isInvalid}>
              <FormLabel>Password</FormLabel>
              <Input
                type="password"
                name="password"
                value={password}
                onChange={handleInputChange}
              />
              <FormErrorMessage>{errorMessage}</FormErrorMessage>
            </FormControl>
          </form>
        </DrawerBody>
      </DrawerContent>
    </Drawer>
  );
}