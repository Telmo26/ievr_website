import CharacterList from "./components/CharacterList";

export default function App() {
  return (
    <div style={{ padding: "2rem", fontFamily: "sans-serif" }}>
      <h1>Game Characters</h1>
      <CharacterList />
    </div>
  );
}