import {  useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { message } from "@tauri-apps/plugin-dialog";

import { Button, TextField, UploadFile } from "../../components";

import styles from "./styles.module.css";

export function CookiesPage(): JSX.Element {
    const [filePath, setFilePath] = useState<string | null>(null);
    const [cookiesText, setCookiesText] = useState<string>("");
    const [loading, setLoading] = useState<boolean>(false);

    const handleSubmit = async (event: React.FormEvent<HTMLFormElement>): Promise<void> => {
        event.preventDefault();

        try {
            setLoading(true);

            let path: string = "";
            if (cookiesText !== "") {
                path = await invoke<string>("save_text_cookies", { text: cookiesText })
            } else if (filePath !== "") {
                path = await invoke<string>("save_json_cookies", { filePath });
            }

            await message(`Salvo em ${path}`, { title: "Cookies salvo com sucesso!", kind: "info" });
        } catch (error) {
            console.error('Error save cookies: ', error);
        } finally {
            setLoading(false);
            setFilePath(null);
            setCookiesText("");
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
                        value: cookiesText,
                        onChange: (e) => setCookiesText(e.target.value.trim()),
                        placeholder: 'Exemplo:\n\n{"url": "https://sigef.incra.gov.br", "cookies": [{"domain": ".incra.gov.br"...}]}',
                        cols: 30,
                        rows: 5,
                    }}
                />
                <p className={styles.or}>Ou</p>
                <UploadFile
                    accept=".json"
                    filePath={filePath}
                    setFilePath={setFilePath}
                />
                <Button
                    title={loading ? "Salvando..." : "Salvar"}
                    type='submit'
                    disabled={loading}
                />
            </form>
        </div>
    );
}
