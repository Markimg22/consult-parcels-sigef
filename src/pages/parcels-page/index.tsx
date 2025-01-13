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

export function ParcelsPage(): JSX.Element {
    const [codeParcelsText, setCodeParcelsText] = useState<string>("");
    const [resultParcelsText, setResultParcelsText] = useState<string>("");

    const [isConsulting, setIsConsulting] = useState<boolean>(false);
    const [isPaused, setIsPaused] = useState<boolean>(false);

    const [loadedCount, setLoadedCount] = useState<number>(0);
    const [totalCount, setTotalCount] = useState<number>(0);

    useEffect(() => {
        let isMounted = true;

        const setupListener = async () => {
            const unlisten = await listen<ParcelData>("consult_parcel_result", async (event) => {
                if (isMounted) {
                    const result = event.payload;

                    if (typeof result === "string") {
                        await message(result, { title: "Houve um erro!", kind: "error" });
                    } else {
                        setResultParcelsText(prev => prev + `${result.parcel_code} | ${result.owner_name} | ${result.owner_cpf_or_cnpj} | ${result.denomination} | ${result.area} | ${result.situation_parcel} | ${result.technical_manager} | ${result.situation_area} | ${result.city_uf} | ${result.registry_office} | ${result.cns} | ${result.registration} | ${result.registration_situation} | ${result.code_incra} | ${result.property_type} | ${result.date_of_entry} | ${result.rt_document}\n`);
                        setLoadedCount(prev => prev + 1);
                    }
                }
            });

            return () => {
                isMounted = false;
                unlisten();
            };
        };

        const cleanup = setupListener();

        return () => {
            cleanup.then((unlisten) => unlisten());
        };
    }, []);

    const handleSubmit = async (event: React.FormEvent<HTMLFormElement>): Promise<void> => {
        event.preventDefault();

        try {
            setIsConsulting(true);

            const parcels = codeParcelsText.trim().split("\n");

            setLoadedCount(0);
            setTotalCount(parcels.length);

            await invoke<ParcelData>("consult_parcels", { parcels });
        } catch (error) {
            console.error('Error consult parcels: ', error);
            await message(String(error).split(".")[0], { title: "Houve um erro!", kind: "error" });
        } finally {
            setIsConsulting(false);
        }
    };

    const handlePauseAndResumeConsult = (): void => {
        setIsPaused(!isPaused);
    }

    const handleCopyResult = (): void => {

    }

    return (
        <div className={styles.container}>
            <form onSubmit={handleSubmit} className={styles.form}>
                <TextField
                    label="Insira os códigos das parcelas"
                    textareaProps={{
                        id: "textarea-code",
                        placeholder: 'Coloque os códigos um em cada linha, exemplo: \n\n#######-#####-####-####\n#######-#####-####-####',
                        value: codeParcelsText,
                        onChange: (e) => setCodeParcelsText(e.target.value.trim()),
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
                    {isConsulting &&  (
                        <Button title={isPaused ? "Continuar" : "Pausar"} variant="outlined" onClick={handlePauseAndResumeConsult} />
                    )}
                    {resultParcelsText !== "" && (
                        <Button type="button" title="Copiar Resultado" variant="outlined" onClick={handleCopyResult} />
                    )}
                    <Button type="submit" disabled={isConsulting || codeParcelsText.trim() === ""} title="Consultar" />
                </div>
            </form>
            <div className={styles.loadingContainer}>
                <div className={styles.progressBarContainer}>
                    <div
                        className={styles.progressBar}
                        style={{ width: `${(loadedCount / totalCount) * 100}%` }}
                        id="progress-bar"
                    ></div>
                </div>
                <p className={styles.loadingLabel}>
                    <span id="loaded">{loadedCount}</span> | <span id="total">{totalCount}</span>
                </p>
            </div>
        </div>
    );
}
