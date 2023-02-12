/* eslint-disable import/no-relative-packages */
import { Scale } from 'chroma-js';

import { color } from '../../src/system/reference';
import styles from './page.module.css';

function ColorChips({ colorScale }: { colorScale: Scale }) {
    const colors = colorScale.colors(11);

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
                <ColorChips colorScale={color.red} />
                <ColorChips colorScale={color.sunset} />
                <ColorChips colorScale={color.orange} />
                <ColorChips colorScale={color.amber} />
                <ColorChips colorScale={color.yellow} />
                <ColorChips colorScale={color.citron} />
                <ColorChips colorScale={color.lime} />
                <ColorChips colorScale={color.green} />
                <ColorChips colorScale={color.mint} />
                <ColorChips colorScale={color.cyan} />
                <ColorChips colorScale={color.sky} />
                <ColorChips colorScale={color.blue} />
                <ColorChips colorScale={color.indigo} />
                <ColorChips colorScale={color.purple} />
                <ColorChips colorScale={color.pink} />
                <ColorChips colorScale={color.rose} />
            </div>
        </main>
    );
}
