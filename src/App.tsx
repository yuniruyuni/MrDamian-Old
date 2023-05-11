import ReactFlow, {
  MiniMap,
  Controls,
  Background,
} from 'reactflow';

import 'reactflow/dist/style.css';

import { usePipeline } from "./pipeline";

function App() {
  const { onApply, ...pipeline } = usePipeline();
  return (
    <div className="container">
      <button title="apply" onClick={onApply} />
      <ReactFlow {...pipeline} >
        <MiniMap />
        <Controls />
        <Background />
      </ReactFlow>
    </div>
  );
}

export default App;
