import * as React from "react";

const Demo = React.lazy(() => import("./Demo"));

const App: React.FC = () => {

    // Render loading screen, or demo
    return (
        <div>
            <React.Suspense fallback={<p>Loading...</p>}>
                <Demo/>
            </React.Suspense>
        </div>
    );
};

export default App;
