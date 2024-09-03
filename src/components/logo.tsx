import { Flex, Heading, useColorMode, useToken } from "@chakra-ui/react";
import { Wallet01 } from "@untitled-ui/icons-react";

function Logo() {
  const { colorMode } = useColorMode();
  const [darkFill, lightFill] = useToken("colors", ["white", "black"]);
  return (
    <Flex>
      <Heading size="2" pr={2}>
        Dev
      </Heading>{" "}
      <Wallet01 />
    </Flex>
  );
}

export default Logo;