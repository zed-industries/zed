/* eslint-disable import/no-relative-packages */
import { color } from '../../src/system/system';
import styles from './page.module.css';

function ColorChips({ colors }: { colors: string[] }) {
    return (
        <div
            style={{
                display: 'flex',
                flexDirection: 'column',
                alignItems: 'center',
                justifyContent: 'center',
                gap: '1px',
            }}
        >
            {colors.map((c) => (
                <div
                    key={c}
                    style={{
                        backgroundColor: c,
                        width: '80px',
                        height: '40px',
                    }}
                    className={styles.chip}
                >
                    {c}
                </div>
            ))}
        </div>
    );
}

export default function Home() {
    return (
        <main>
            <div style={{ display: 'flex', gap: '1px' }}>
                <ColorChips colors={color.red} />
                <ColorChips colors={color.sunset} />
                <ColorChips colors={color.orange} />
                <ColorChips colors={color.amber} />
                <ColorChips colors={color.yellow} />
                <ColorChips colors={color.citron} />
                <ColorChips colors={color.lime} />
                <ColorChips colors={color.green} />
                <ColorChips colors={color.mint} />
                <ColorChips colors={color.cyan} />
                <ColorChips colors={color.sky} />
                <ColorChips colors={color.blue} />
                <ColorChips colors={color.indigo} />
                <ColorChips colors={color.purple} />
                <ColorChips colors={color.pink} />
                <ColorChips colors={color.rose} />
            </div>
        </main>
    );
}
