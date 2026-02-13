import type { Character } from "../types/character";

export async function fetchCharacters(): Promise<Character[]> {
  console.log("fetchCharacters called");
  const res = await fetch("http://localhost:3000/api/characters/summary");
  if (!res.ok) throw new Error("Failed to fetch characters");
  return res.json();
}