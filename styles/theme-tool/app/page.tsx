/* eslint-disable import/no-relative-packages */

import * as color from '../../src/system/ref/color';
import { ColorFamily } from '../../src/system/types';
import styles from './page.module.css';

function ColorChips({ colorFamily }: { colorFamily: ColorFamily }) {
    const familySubset = [0, 11, 22, 33, 44, 56, 67, 79, 90, 101];

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
            <div
                style={{
                    fontFamily: 'monospace',
                    fontSize: '12px',
                    fontWeight: 'bold',
                    padding: '16px 0',
                }}
            >
                {colorFamily.name}
            </div>
            {colorFamily.scale.colors.map(
                (c) =>
                    familySubset.includes(c.step) && (
                        <div
                            key={c.step}
                            style={{
                                backgroundColor: c.hex,
                                color: c.isLight ? 'black' : 'white',
                                width: '80px',
                                height: '40px',
                            }}
                            className={styles.chip}
                        >
                            {c.hex}
                        </div>
                    ),
            )}
        </div>
    );
}

export default function Home() {
    return (
        <main>
            <div style={{ display: 'flex', gap: '1px' }}>
                <ColorChips colorFamily={color.lightgray} />
                <ColorChips colorFamily={color.darkgray} />
                <ColorChips colorFamily={color.red} />
                <ColorChips colorFamily={color.sunset} />
                <ColorChips colorFamily={color.orange} />
                <ColorChips colorFamily={color.amber} />
                <ColorChips colorFamily={color.yellow} />
                <ColorChips colorFamily={color.lemon} />
                <ColorChips colorFamily={color.citron} />
                <ColorChips colorFamily={color.lime} />
                <ColorChips colorFamily={color.green} />
                <ColorChips colorFamily={color.mint} />
                <ColorChips colorFamily={color.cyan} />
                <ColorChips colorFamily={color.sky} />
                <ColorChips colorFamily={color.blue} />
                <ColorChips colorFamily={color.indigo} />
                <ColorChips colorFamily={color.purple} />
                <ColorChips colorFamily={color.pink} />
                <ColorChips colorFamily={color.rose} />
            </div>
        </main>
    );
}
