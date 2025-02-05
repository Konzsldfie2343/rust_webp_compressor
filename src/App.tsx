import { useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import "./App.css";

const App = (): JSX.Element => {
  const [folderPath, setFolderPath] = useState<string>("");
  const [result, setResult] = useState<string>("");

  const handleSelectFolder = useCallback(async () => {
    try {
      const selectedPath = await open({ directory: true });
      if (typeof selectedPath === "string") {
        setFolderPath(selectedPath);
        const response = await invoke<string>("convert_folder_to_webp", { folderPath: selectedPath });
        setResult(response);
      }
    } catch (error) {
      setResult("エラー: " + error);
    }
  }, []);

  return (
    <div style={{ textAlign: "center", padding: "2rem" }}>
      <h1>画像WebP変換アプリ</h1>
      <button onClick={handleSelectFolder} style={{ marginBottom: "1rem" }}>
        フォルダを選択
      </button>
      {folderPath && <p>対象フォルダ: {folderPath}</p>}
      {result && <p>{result}</p>}
    </div>
  );
};

export default App;
