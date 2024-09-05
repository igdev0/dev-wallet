import { BrowserRouter, Route, Routes } from "react-router-dom";
import CreateWalletScreen from "./screens/create-wallet.tsx";
import { useAppState } from "./state.ts";
import AuthenticateScreen from "./screens/authenticate.tsx";
import AccountsScreen from "./screens/accounts.tsx";

function App() {
  const appState = useAppState();

  return (
    <BrowserRouter>
      <Routes>
        <Route index={true} path="/" element={<CreateWalletScreen />} />
        <Route path="/authenticate" element={<AuthenticateScreen />} />
        <Route path="/accounts/:wallet_id" element={<AccountsScreen />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;