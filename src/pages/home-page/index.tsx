import { useState } from "react";

import { CookiesPage, ParcelsPage } from "../../pages";
import packageJson from '../../../package.json';

import styles from './styles.module.css';

enum Pages {
    PARCELS = "PARCELS",
    COOKIES = "COOKIES"
}

export function HomePage(): JSX.Element {
    const [pages, setPages] = useState<Pages>(Pages.PARCELS);

    const handleChangePage = (page: Pages): void => {
        setPages(page);
    }

    return (
        <main className={styles.container}>
            <h1 className={styles.title}>
                Consultar Parcelas SIGEF <span className={styles.version}>v{packageJson.version}</span>
            </h1>
            <nav>
                <ul className={styles.navbar}>
                    <li className={styles.link} onClick={() => handleChangePage(Pages.PARCELS)}>
                        Parcelas
                    </li>
                    <li className={styles.link} onClick={() => handleChangePage(Pages.COOKIES)}>
                        Cookies
                    </li>
                </ul>
            </nav>
            <hr className={styles.line} />
            <div className={styles.content}>
                <div style={{ display: pages === Pages.PARCELS ? "block" : "none" }}>
                    <ParcelsPage />
                </div>
                <div style={{ display: pages === Pages.COOKIES ? "block" : "none" }}>
                    <CookiesPage />
                </div>
            </div>
        </main>
    );
}
