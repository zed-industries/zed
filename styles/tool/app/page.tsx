import { useTheme } from "./useTheme";

export default function Page() {
    const theme = useTheme();

    const button = theme.ui.find.case_button;

    return (
        <div>
            <h1>Hello, Next.js!</h1>
            <div>
                <button type="button"
                    style={{ backgroundColor: 'red', color: 'white', padding: '10px', borderRadius: '5px' }}>
                    Case
                </button>
            </div>
        </div>
    );
}
