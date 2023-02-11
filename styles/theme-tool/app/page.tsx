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
                <ColorChips colorScale={color.grayLight} />
                <ColorChips colorScale={color.roseLight} />
                <ColorChips colorScale={color.redLight} />
                <ColorChips colorScale={color.orangeLight} />
                <ColorChips colorScale={color.amberLight} />
                <ColorChips colorScale={color.yellowLight} />
                <ColorChips colorScale={color.limeLight} />
                <ColorChips colorScale={color.greenLight} />
                <ColorChips colorScale={color.emeraldLight} />
                <ColorChips colorScale={color.jadeLight} />
                <ColorChips colorScale={color.tealLight} />
                <ColorChips colorScale={color.cyanLight} />
                <ColorChips colorScale={color.lightBlueLight} />
                <ColorChips colorScale={color.blueLight} />
                <ColorChips colorScale={color.indigoLight} />
                <ColorChips colorScale={color.violetLight} />
                <ColorChips colorScale={color.pinkLight} />
                <ColorChips colorScale={color.brownLight} />
            </div>
        </main>
    );
}
