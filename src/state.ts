import { create } from "zustand";

interface AppState {
  wallet_authenticated: string | null;
  setAuthenticated: (name: string) => void;
}

const useAppState = create<AppState>((setState, getState, store) => ({
  wallet_authenticated: null,
  setAuthenticated(wallet_name: string) {
    setState({ wallet_authenticated: wallet_name });
  },
  logout() {
    setState({ wallet_authenticated: null });
  },
}));