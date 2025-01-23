import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { message } from "@tauri-apps/plugin-dialog";
import { listen } from "@tauri-apps/api/event";

import { Button, TextField } from "../../components";

import styles from "./styles.module.css";

type ParcelData = {
    parcel_code: string;
    owner_name: string;
    owner_cpf_or_cnpj: string;
    denomination: string;
    area: string;
    situation_parcel: string;
    technical_manager: string;
    situation_area: string;
    city_uf: string;
    registry_office: string;
    cns: string;
    registration: string;
    registration_situation: string;
    code_incra: string;
    property_type: string;
    date_of_entry: string;
    rt_document: string;
}

type ParcelResponse = {
    data: ParcelData;
    total_count: number;
    current_count: number;
}

type ConsultStatus = "default" | "consulting" | "paused";

export function ParcelsPage(): JSX.Element {
    const [codeParcelsText, setCodeParcelsText] = useState<string>("");
    const [resultParcelsText, setResultParcelsText] = useState<string>("");

    const [loadedCount, setLoadedCount] = useState<number>(0);
    const [totalCount, setTotalCount] = useState<number>(0);

    const [consultStatus, setConsultStatus] = useState<ConsultStatus>("default");

    useEffect(() => {
        const unlistenSuccess = listen<ParcelResponse | string>("consult_parcels", async (event) => {
            if (typeof event.payload === "string") {
                setConsultStatus("paused");
                await message(event.payload, { title: "Houve um erro!", kind: "error" });
            } else {
                const result = event.payload;

                setLoadedCount(result.current_count);

                setResultParcelsText(prev =>
                    prev + `${result.data.parcel_code} | ${result.data.owner_name} | ${result.data.owner_cpf_or_cnpj} | ${result.data.denomination} | ${result.data.area} | ${result.data.situation_parcel} | ${result.data.technical_manager} | ${result.data.situation_area} | ${result.data.city_uf} | ${result.data.registry_office} | ${result.data.cns} | ${result.data.registration} | ${result.data.registration_situation} | ${result.data.code_incra} | ${result.data.property_type} | ${result.data.date_of_entry} | ${result.data.rt_document}\n`
                );
            }
        });

        return () => {
            unlistenSuccess.then((fn) => fn());
        };
    }, []);

    const handleSubmit = async (event: React.FormEvent<HTMLFormElement>): Promise<void> => {
        event.preventDefault();
        console.log("Submit");

        try {
            setConsultStatus("consulting");

            const parcels = codeParcelsText.split("\n").map(item => item.trim());
            setTotalCount(parcels.length);

            await invoke("reset_consult");
            await invoke<ParcelResponse>("consult_parcels", { parcels });
        } catch (error) {
            console.error('Error consult parcels: ', error);
        }
    };

    const handleReset = async (): Promise<void> => {
        console.log("Reset");

        try {
            await invoke("cancel_consult");

            setTotalCount(0);
            setLoadedCount(0);
            setCodeParcelsText("");
            setResultParcelsText("");
            setConsultStatus("default");
        } catch (error) {
            console.error("Error cancel consult", error);
        }
    };

    const handlePauseConsult = async (): Promise<void> => {
        console.log("Pause");

        try {
            setConsultStatus("paused");
            await invoke("pause_consult");
        } catch (error) {
            console.error("Error pause consult", error);
        }
    };

    const handleResumeConsult = async (): Promise<void> => {
        console.log("Resume");

        try {
            setConsultStatus("consulting");
            await invoke("resume_consult");
        } catch (error) {
            console.error("Error resume consult", error);
        }
    };

    const handleCopyResult = async (): Promise<void> => {
        try {
            await navigator.clipboard.writeText(resultParcelsText);
            await message("Resultado copiado com sucesso!", { title: "Sucesso!", kind: "info" });
        } catch (error) {
            console.error("Error copying result: ", error);
            await message("Erro ao copiar resultado", { title: "Erro!", kind: "error" });
        }
    };

    return (
        <div className={styles.container}>
            <form onSubmit={handleSubmit} className={styles.form}>
                <TextField
                    label="Insira os códigos das parcelas"
                    textareaProps={{
                        id: "textarea-code",
                        placeholder: 'Coloque os códigos um em cada linha, exemplo: \n\n#######-#####-####-####\n#######-#####-####-####',
                        value: codeParcelsText,
                        onChange: (e) => setCodeParcelsText(e.target.value),
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
                            value: resultParcelsText,
                            disabled: true,
                            cols: 30,
                            rows: 10
                        }}
                    />
                </div>
                <p className={styles.orderData}>
                    <strong>Ordem dos dados:</strong><br />
                    Código da Parcela | Nome Proprietário | CPF ou CNPJ do Proprietário | Denominação | Área | Situação Parcela | Responsável Técnico | Situação Área | Cidade-UF | Cartório | CNS | Matrícula | Situação Matrícula | Código INCRA | Tipo da Propriedade | Data de Entrada | Documento RT
                </p>
                <div className={styles.buttonsContainer}>
                    {consultStatus === "consulting" &&  (
                        <Button type="button" title="Pausar" variant="outlined" onClick={handlePauseConsult} />
                    )}
                    {consultStatus === "paused" && (
                        <Button type="button" title="Continuar" variant="outlined" onClick={handleResumeConsult} />
                    )}
                    {resultParcelsText.trim() !== "" && (
                        <Button type="button" title="Copiar Resultado" variant="outlined" onClick={handleCopyResult} />
                    )}
                    {(consultStatus === "consulting" || consultStatus === "paused") && (
                        <Button type="button" title="Resetar" onClick={handleReset} />
                    )}
                    {consultStatus === "default" && (
                        <Button type="submit" disabled={codeParcelsText.trim() === ""} title="Consultar" />
                    )}
                </div>
            </form>
            {(consultStatus === "consulting" || consultStatus === "paused") && (
                <div className={styles.loadingContainer}>
                    <div className={styles.progressBarContainer}>
                        <div
                            className={`${styles.progressBar} ${consultStatus === "paused" ? styles.paused : ''}`}
                            style={{ width: `${(loadedCount / totalCount) * 100}%` }}
                            id="progress-bar"
                        ></div>
                    </div>
                    <p className={styles.loadingLabel}>
                        <span id="loaded">{loadedCount}</span> | <span id="total">{totalCount}</span>
                        {consultStatus === "paused" && <span className={styles.pausedText}> (Pausado)</span>}
                    </p>
                </div>
            )}
        </div>
    );
}
