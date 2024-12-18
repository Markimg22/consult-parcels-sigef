import { ButtonHTMLAttributes } from "react";

import styles from "./styles.module.css";

type Props = ButtonHTMLAttributes<HTMLButtonElement> & {
    title: string;
    variant?: 'contained' | 'outlined';
};

export function Button({ title, variant = 'contained', ...rest }: Props): JSX.Element {
    return (
        <button
            className={styles[variant]}
            {...rest}
        >
            {title}
        </button>
    );
}
