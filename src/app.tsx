import { BrowserRouter, Route, Routes } from "react-router-dom";
import MnemonicScreen from "./screens/create-wallet.tsx";
import { useAppState } from "./state.ts";
import AuthenticateScreen from "./screens/authenticate.tsx";

function App() {
  const appState = useAppState();

  return (
    <BrowserRouter>
      <Routes>
        <Route
          index={true}
          path="/authenticate"
          element={<AuthenticateScreen />}
        />
        <Route index={true} path="/" element={<MnemonicScreen />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;