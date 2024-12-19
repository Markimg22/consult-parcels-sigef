import { useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

import { Button, TextField, UploadField } from "../../components";

import styles from "./styles.module.css";

export function CookiesPage(): JSX.Element {
    const [file, setFile] = useState<File | null>(null);

    const textareaCookiesRef = useRef<HTMLTextAreaElement>(null);
    const fileInputRef = useRef<HTMLInputElement>(null);

    const handleSubmit = async (event: React.FormEvent<HTMLFormElement>): Promise<void> => {
        event.preventDefault();

        try {
            const message = await invoke("greet");
            console.log(message);
        } catch (error) {
            console.error(error);
        }
    };

    return (
        <div className={styles.container}>
            <form onSubmit={handleSubmit} className={styles.form}>
                <TextField
                    label='Insira os Cookies da plataforma Consulta SIGEF'
                    labelProps={{
                        htmlFor: 'textarea-cookies'
                    }}
                    textareaProps={{
                        id: 'textarea-cookies',
                        ref: textareaCookiesRef,
                        placeholder: 'Exemplo:\n\n{"url": "https://sigef.incra.gov.br", "cookies": [{"domain": ".incra.gov.br"...}]}',
                        cols: 30,
                        rows: 5,
                    }}
                />
                <p className={styles.or}>Ou</p>
                <UploadField
                    accept=".json"
                    fileInputRef={fileInputRef}
                    file={file}
                    setFile={setFile}
                />
                <Button
                    title="Salvar"
                    type='submit'
                />
            </form>
        </div>
    );
}
