import * as React from "react";

import { __wbg_set_wasm } from "../pkg/index_bg.js";
export * from "../pkg/index_bg.js";

import wasmModulePromise from "./wasm_promise.js";
import { useEffect, useRef, useState } from "react";

import { sizeFormatter } from "human-readable"

import { Card, CardContent, Typography, Container, Stack, Slider, Grid, ToggleButtonGroup, ToggleButton } from "@mui/material";
import { Box } from "@mui/system";

import '@fontsource/roboto/300.css';
import '@fontsource/roboto/400.css';
import '@fontsource/roboto/500.css';
import '@fontsource/roboto/700.css';

const Demo: React.FC = () => {

    const [wasmModule, setWasmModule] = useState(null);
    const [wasmData, setWasmData] = useState(null);

    useEffect(() => {
        const loadWasmModule = async () => {
            const wasm = await wasmModulePromise;

            console.log("module", wasm);

            __wbg_set_wasm(wasm);

            wasm.__wbindgen_start();

            setWasmData(wasm);
            setWasmModule(module);
        };

        loadWasmModule();
    }, []);

    const [frameRate, setFramerate] = useState<string>("N/A");
    const [averageCompare, setAverageCompare] = useState<string>("N/A");

    useEffect(() => {
        function handleCustomEvent(event: any) {
            const data = JSON.parse(event.detail);

            setFramerate(`${data.frame_rate} FPS`);
            setAverageCompare(`${Math.round(data.average_compare_against * 100) / 100} boids`);
        }

        document.addEventListener('quadtree_update_stats', handleCustomEvent);
        return () => {
            document.removeEventListener('quadtree_update_stats', handleCustomEvent);
        };
    }, []);


    const data = (() => {
        if (wasmData != null) {
            const format = sizeFormatter({
                std: 'JEDEC',
                decimalPlaces: 2,
                keepTrailingZeroes: false,
            })

            return format(wasmData.memory.buffer.byteLength) as string;
        }
        else {
            return "N/A";
        }
    })();

    const [wasmValues, setWasmValues] = useState({
        dampening: 0.955,
        boid_count: 50,
        selected_line_alpha: 1.0,
        unselected_lines_alpha: 1.0,
        selected_quads: 0.3,
        unselected_quads: 0.05,
        lookup_radius: 100,
        lookup_alpha: 0.5,
        boid_size: 5.0,
        lookup_method: "quadtree"
    });

    const handleWasmValuesUpdate = (key: string, value: number | number[]) => {
        setWasmValues({ ...wasmValues, [key]: value });

        if (value != null) {
            window.dispatchEvent(new CustomEvent('wasmevent', {
                detail: {
                    event_type: 'set_' + key,
                    data: value.toString()
                }
            }));
        }
    }

    useEffect(() => {
        for (const [key, value] of Object.entries(wasmValues)) {
            window.dispatchEvent(new CustomEvent('wasmevent', {
                detail: {
                    event_type: 'set_' + key,
                    data: value.toString()
                }
            }));
        }
    }, [wasmModule]);

    const myElementRef = useRef(null);

    const statBlock = (() => {
        if (wasmData == null) {
            return <div>Loading...</div>;
        }
        else {
            return (<>

                <Grid container spacing={2}>
                    <Grid item xs={12}>
                        <Typography fontWeight={600}>Performance</Typography>
                    </Grid>
                    <Grid item xs={4}>
                        <Typography>Target boid count</Typography>
                    </Grid>
                    <Grid item xs={8}>
                        <Slider value={wasmValues.boid_count} onChange={(evt, val) => handleWasmValuesUpdate("boid_count", val)} min={25} max={1000} step={1} valueLabelDisplay="auto" />
                    </Grid>
                    <Grid item xs={4}>
                        <Typography>Lookup Radius</Typography>
                    </Grid>
                    <Grid item xs={8}>
                        <Slider value={wasmValues.lookup_radius} onChange={(evt, val) => handleWasmValuesUpdate("lookup_radius", val)} min={10} max={1000} step={1.0} valueLabelDisplay="auto" />
                    </Grid>

                    <Grid item xs={4}>
                        <Typography>Lookup method</Typography>
                    </Grid>
                    <Grid item xs={8}>
                        <ToggleButtonGroup
                            color="primary"
                            value={wasmValues.lookup_method}
                            exclusive
                            onChange={(evt, val) => handleWasmValuesUpdate("lookup_method", val)}
                            aria-label="Lookup method"
                        >
                            <ToggleButton value="quadtree">Quadtree</ToggleButton>
                            <ToggleButton value="brute_force">Brute-force</ToggleButton>
                        </ToggleButtonGroup>
                    </Grid>



                    <Grid item xs={4}>
                        <Typography>Average number of boid sqrt function's per frame</Typography>
                    </Grid>

                    <Grid item xs={8}>
                        <Typography>{averageCompare}</Typography>
                    </Grid>

                    <Grid item xs={12}>
                        <Typography fontWeight={600}>Physics</Typography>
                    </Grid>
                    <Grid item xs={4}>
                        <Typography>Dampening</Typography>
                    </Grid>
                    <Grid item xs={8}>
                        <Slider value={wasmValues.dampening} onChange={(evt, val) => handleWasmValuesUpdate("dampening", val)} min={0.8} max={1} step={0.001} valueLabelDisplay="auto" />
                    </Grid>

                    <Grid item xs={12}>
                        <Typography fontWeight={600}>Aesthetics</Typography>
                    </Grid>

                    <Grid item xs={4}>
                        <Typography>Selected Lines</Typography>
                    </Grid>
                    <Grid item xs={8}>
                        <Slider value={wasmValues.selected_line_alpha} onChange={(evt, val) => handleWasmValuesUpdate("selected_line_alpha", val)} min={0} max={1} step={0.01} valueLabelDisplay="auto" />
                    </Grid>

                    <Grid item xs={4}>
                        <Typography>Unselected Lines</Typography>
                    </Grid>
                    <Grid item xs={8}>
                        <Slider value={wasmValues.unselected_lines_alpha} onChange={(evt, val) => handleWasmValuesUpdate("unselected_lines_alpha", val)} min={0} max={1} step={0.001} valueLabelDisplay="auto" />
                    </Grid>

                    <Grid item xs={4}>
                        <Typography>Selected Quads</Typography>
                    </Grid>
                    <Grid item xs={8}>
                        <Slider value={wasmValues.selected_quads} onChange={(evt, val) => handleWasmValuesUpdate("selected_quads", val)} min={0} max={1} step={0.01} valueLabelDisplay="auto" />
                    </Grid>

                    <Grid item xs={4}>
                        <Typography>Unselected Quads</Typography>
                    </Grid>
                    <Grid item xs={8}>
                        <Slider value={wasmValues.unselected_quads} onChange={(evt, val) => handleWasmValuesUpdate("unselected_quads", val)} min={0} max={1} step={0.01} valueLabelDisplay="auto" />
                    </Grid>

                    <Grid item xs={4}>
                        <Typography>Lookup Alpha</Typography>
                    </Grid>
                    <Grid item xs={8}>
                        <Slider value={wasmValues.lookup_alpha} onChange={(evt, val) => handleWasmValuesUpdate("lookup_alpha", val)} min={0} max={1} step={0.01} valueLabelDisplay="auto" />
                    </Grid>

                    <Grid item xs={4}>
                        <Typography>Boid Size</Typography>
                    </Grid>
                    <Grid item xs={8}>
                        <Slider value={wasmValues.boid_size} onChange={(evt, val) => handleWasmValuesUpdate("boid_size", val)} min={0} max={20} step={0.1} valueLabelDisplay="auto" />
                    </Grid>

                    <Grid item xs={4}>
                        <Typography>Frame Rate</Typography>
                    </Grid>
                    <Grid item xs={8}>
                        <Typography>{frameRate}</Typography>
                    </Grid>
                    <Grid item xs={4}>
                        <Typography>WASM Buffer Size</Typography>
                    </Grid>
                    <Grid item xs={8}>
                        <Typography>{data}</Typography>
                    </Grid>
                </Grid>
            </>)
        }
    })();

    return (
        <Container maxWidth="lg">
            <h1>Quadtree in Rust WASM</h1>

            <Card>
                <CardContent>
                    <Stack spacing={2} direction="column">
                        <Typography>
                            Below is an interactive demonstration of the benefits of the quadtree data structure. It's used to efficently structure 2D data. In our case, we have a list of Boids (points on a 2D canvas with a target & velocity) and want to render a line between nearby Boids when they're close to each other. If we were to compare each Boid against every other Boid, we'd have a time complexity of O(n^2). Given we are performing an expensive square-root function on each Boid we compare with (to calculate the distance for the correct alpha for the line), we'd like to reduce that. Luckily with the quadtree, we can reduce this to O(n log n).
                        </Typography>
                        <Typography>
                            A quadtree is a recursive structure. Starting with a single root quadtree, when we insert a Boid into the Quadtree it will try to place it in a bucket but if there is no space, the quad will subdivide into four new quadtree structures, insert all it's own children into the new child quadtrees, then try to place the intended Boid again within one of it's children, recursively.
                        </Typography>
                        <Typography>
                            To lookup nearby Boids, starting from the root of the quadtree we check if this level's boundary intersects with the quad, if it does, we check the children, and so on. When we find a leaf that intersects with the lookup radius, we add the leaf's Boid to a list of nearby Boids.
                        </Typography>
                        <Typography>The larger red Boid will follow your cursor, and you can touch the canvas below to move all Boids towards a point (note how the average number of comparisons goes up as they converge). The different levels of the quadtree being used for the red Boid are rendered in red, with Boid's being checked for proximity to the red Boid are also rendered red.</Typography>
                    </Stack>
                </CardContent>
            </Card>

            <Box sx={{ flexGrow: 1, padding: "24px" }}>
                <canvas id="canvas" style={{ minWidth: "100%" }} ref={myElementRef}></canvas>
            </Box>

            <Card>
                <CardContent>
                    {statBlock}
                </CardContent>
            </Card>
        </Container>
    );
};

export default Demo;
