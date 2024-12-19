import { ChangeEvent, InputHTMLAttributes, RefObject, useCallback } from "react";

import { Button } from "../button";

import styles from "./styles.module.css";

type Props = InputHTMLAttributes<HTMLInputElement> & {
    file: File | null;
    setFile: (file: File | null) => void;
    fileInputRef: RefObject<HTMLInputElement>;
};

export function UploadField({file, setFile, fileInputRef, ...rest}: Props): JSX.Element {
    const handleFileChange = useCallback((event: ChangeEvent<HTMLInputElement>) => {
        if (event.target.files && event.target.files[0]) {
            setFile(event.target.files[0]);
        }
    }, []);

    const handleClick = useCallback(() => {
        if (fileInputRef.current) {
            fileInputRef.current.click();
        }
    }, []);

    const handleRemoveFile = useCallback(() => {
        setFile(null);

        if (fileInputRef.current) {
            fileInputRef.current.value = "";
        }
    }, []);

    return (
        <div className={styles.container}>
            {file && <Button onClick={handleRemoveFile} variant="outlined" title="Remover arquivo selecionado" />}
            <div
                className={`${styles.inputContainer} ${file ? styles.borderSolid : styles.borderDashed}`}
                onClick={handleClick}
                onChange={handleFileChange}
            >
                {file
                    ? <p className={styles.text}>Arquivo selecionado: <strong>{file.name}</strong>, clique para substituir por outro arquivo.</p>
                    : <p className={styles.text}>Clique para selecionar um arquivo .json</p>
                }
                <input
                    type="file"
                    className={styles.input}
                    ref={fileInputRef}
                    {...rest}
                />
            </div>
        </div>
    );
}
