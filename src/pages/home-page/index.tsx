import { useState } from "react";

import { CookiesPage, ParcelsPage } from "../../pages";

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
            <h1 className={styles.title}>Consultar Parcelas SIGEF</h1>
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
                {pages === Pages.PARCELS ? <ParcelsPage /> : <CookiesPage />}
            </div>
        </main>
    );
}
