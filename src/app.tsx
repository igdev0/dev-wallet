import { BrowserRouter, Route, Routes } from "react-router-dom";
import MnemonicScreen from "./screens/create-wallet.tsx";
import CredentialsScreen from "./screens/credentials.tsx";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route index={true} path="/" element={<MnemonicScreen />} />
        <Route path="/security" element={<CredentialsScreen />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;