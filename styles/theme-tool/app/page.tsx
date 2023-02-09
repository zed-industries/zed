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
    const { red, gray, rose } = color;
    return (
        <main>
            <div style={{ display: 'flex', gap: '1px' }}>
                <ColorChips colorScale={gray} />
                <ColorChips colorScale={rose} />
                <ColorChips colorScale={red} />
            </div>
        </main>
    );
}
