import { Link as ReactRouterLink } from "react-router-dom";
import { PropsWithChildren } from "react";
import { Link as ChakraLink, LinkProps } from "@chakra-ui/react";

interface InternalLinkProps extends LinkProps, PropsWithChildren {
  to: string;
}

export default function InternalLink({
  children,
  ...props
}: InternalLinkProps) {
  return (
    <ChakraLink as={ReactRouterLink} {...props}>
      {children}
    </ChakraLink>
  );
}