import { useRef } from "react";

import { Button, TextField } from "../../components";

import styles from "./styles.module.css";

export function ParcelsPage(): JSX.Element {
    const textareaCodeRef = useRef<HTMLTextAreaElement>(null);
    const textareaResultRef = useRef<HTMLTextAreaElement>(null);

    const isConsulting = false;

    const handleSubmit = (event: React.FormEvent<HTMLFormElement>): void => {
        event.preventDefault();
        const codes = textareaCodeRef.current?.value;
        console.log(codes);
    };

    return (
        <div className={styles.container}>
            <form onSubmit={handleSubmit} className={styles.form}>
                <TextField
                    label="Insira os códigos das parcelas"
                    textareaProps={{
                        id: "textarea-code",
                        placeholder: 'Coloque os códigos um em cada linha, exemplo: \n\n#######-#####-####-####\n#######-#####-####-####',
                        ref: textareaCodeRef,
                        cols: 30,
                        rows: 10
                    }}
                    labelProps={{
                        htmlFor: 'textarea-code'
                    }}
                />
                <div>
                    <TextField
                        label="Resultado"
                        labelProps={{
                            htmlFor: 'textarea-result'
                        }}
                        textareaProps={{
                            id: 'textarea-result',
                            ref: textareaResultRef,
                            disabled: true,
                            cols: 30,
                            rows: 10
                        }}
                    />
                </div>
                <div className={styles.buttonsContainer}>
                    {isConsulting && (
                        <Button disabled title="Pausar" variant="outlined" />
                    )}
                    <Button title="Copiar Resultado" variant="outlined" />
                    <Button type="submit" title="Consultar" />
                </div>
            </form>
            <div className={styles.loadingContainer}>
                <div className={styles.progressBarContainer}>
                    <div className={styles.progressBar} id="progress-bar"></div>
                </div>
                <p className={styles.loadingLabel}>
                    <span id="loaded">0</span> | <span id="total">0</span>
                </p>
            </div>
        </div>
    );
}
