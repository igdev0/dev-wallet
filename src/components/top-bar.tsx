import { Flex, Heading } from "@chakra-ui/react";

export default function TopBar({ text = "" }) {
  return (
    <Flex
      h="10"
      bgColor="teal.300"
      position="sticky"
      top="0"
      alignItems="center"
    >
      <Heading w="100%" textAlign="center" lineHeight="1" size="sm">
        {text}
      </Heading>
    </Flex>
  );
}