import ReactFlow, {
  MiniMap,
  Controls,
  Background,
  useNodesState,
  useEdgesState,
  addEdge,
  updateEdge
} from 'reactflow';

import 'reactflow/dist/style.css';

import { usePipeline } from "./pipeline";

function App() {
  const pipeline = usePipeline();
  return (
    <div className="container">
      <ReactFlow {...pipeline} >
        <MiniMap />
        <Controls />
        <Background />
      </ReactFlow>
    </div>
  );
}

export default App;
