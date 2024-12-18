import { useRef } from "react";

import { Button, TextField } from "../../components";

import styles from "./styles.module.css";

export function CookiesPage(): JSX.Element {
    const textareaCookiesRef = useRef<HTMLTextAreaElement>(null);

    const handleSubmit = (event: React.FormEvent<HTMLFormElement>): void => {
        event.preventDefault();
    };

    return (
        <div className={styles.container}>
            <form onSubmit={handleSubmit}>
                <TextField
                    label='Insira os Cookies da plataforma de Consulta SIGEF'
                    labelProps={{
                        htmlFor: 'textarea-cookies'
                    }}
                    textareaProps={{
                        id: 'textarea-cookies',
                        ref: textareaCookiesRef,
                        placeholder: 'Exemplo:\n\n[\n{\n"domain": ".incra.gov.br",\n"expirationDate": 1765759556.3249,\n"hostOnly":false,\n"httpOnly":false,\n"name":"_ga",\n...\n},\n...outras chaves\n]',
                        cols: 30,
                        rows: 20
                    }}
                />
                <Button
                    title="Salvar"
                    type='submit'
                />
            </form>
        </div>
    );
}
