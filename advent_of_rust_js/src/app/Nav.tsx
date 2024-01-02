import styles from "./nav.module.css";
import { PuzzleDay } from "./page";

interface NavProps {
    items: PuzzleDay[],
    onClick: (day: PuzzleDay) => void
}

const Nav = (props: NavProps) => {
    return (
        <nav className={styles.nav}>
            <ul>
                {props.items.map(i =>
                    <li key={i} onClick={() => props.onClick(i)}><a>[ {i} ]</a></li>
                )}
            </ul>
        </nav >
    )
}

export default Nav