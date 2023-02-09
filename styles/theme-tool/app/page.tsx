/* eslint-disable import/no-relative-packages */
import { Scale } from 'chroma-js';

import { color } from '../../src/system/reference';

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
                >
                    {c}
                </div>
            ))}
        </div>
    );
}

export default function Home() {
    const {
        grayLight,
        grayDark,
        roseDark,
        roseLight,
        redDark,
        redLight,
        orangeDark,
        orangeLight,
        amberDark,
        amberLight,
    } = color;
    return (
        <main>
            <div style={{ display: 'flex', gap: '1px' }}>
                <ColorChips colorScale={grayLight} />
                <ColorChips colorScale={roseLight} />
                <ColorChips colorScale={redLight} />
                <ColorChips colorScale={orangeLight} />
                <ColorChips colorScale={amberLight} />

                <ColorChips colorScale={grayDark} />
                <ColorChips colorScale={roseDark} />
                <ColorChips colorScale={redDark} />
                <ColorChips colorScale={orangeDark} />
                <ColorChips colorScale={amberDark} />
            </div>
        </main>
    );
}
