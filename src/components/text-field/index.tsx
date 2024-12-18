import { LabelHTMLAttributes, RefAttributes, TextareaHTMLAttributes } from "react";

import styles from "./styles.module.css";

type Props = {
    label: string;
    labelProps?: LabelHTMLAttributes<HTMLLabelElement>;
    textareaProps?: TextareaHTMLAttributes<HTMLTextAreaElement> & RefAttributes<HTMLTextAreaElement>;
};

export function TextField({ label, labelProps, textareaProps }: Props): JSX.Element {
    return (
        <div className={styles.container}>
             <label className={styles.label} {...labelProps}>
                {label}
            </label>
            <textarea
                className={styles.textarea}
                {...textareaProps}
            />
        </div>
    );
}
