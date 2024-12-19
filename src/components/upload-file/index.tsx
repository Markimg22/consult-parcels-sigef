import { InputHTMLAttributes, useCallback } from "react";
import { open } from "@tauri-apps/plugin-dialog";

import { Button } from "../button";

import styles from "./styles.module.css";

type Props = InputHTMLAttributes<HTMLInputElement> & {
    filePath: string | null;
    setFilePath: (file: string | null) => void;
};

export function UploadFile({filePath, setFilePath, ...rest}: Props): JSX.Element {
    const handleClick = async (): Promise<void> => {
        const selected = await open({
            multiple: false,
            filters: [{ name: "JSON Files", extensions: ["json"]}]
        });

        if (selected) {
            setFilePath(selected);
        }
    };

    const handleRemoveFile = useCallback(() => {
        setFilePath(null);
    }, []);

    return (
        <div className={styles.container}>
            {filePath && <Button onClick={handleRemoveFile} variant="outlined" title="Remover arquivo selecionado" />}
            <div
                className={`${styles.inputContainer} ${filePath ? styles.borderSolid : styles.borderDashed}`}
                onClick={handleClick}
            >
                {filePath
                    ? <p className={styles.text}>Arquivo selecionado: <strong>{filePath}</strong>, clique para substituir por outro arquivo.</p>
                    : <p className={styles.text}>Clique para selecionar um arquivo .json</p>
                }
                <input
                    type="file"
                    className={styles.input}
                    {...rest}
                />
            </div>
        </div>
    );
}
