import { useState, useEffect, useCallback, useMemo } from 'react';
import ReactFlow, {
  MiniMap,
  Controls,
  Background,
} from 'reactflow';

import 'reactflow/dist/style.css';

import { Button } from 'semantic-ui-react'

import { usePipeline } from "./pipeline";

import { Node } from './bindings';
import { ContextMenu } from "./ContextMenu";

export type ContextMenuState = {
  open: boolean;
  x: number;
  y: number;
};

function App() {
  const { onApply, addNode, ...pipeline } = usePipeline();

  const [ menu, setMenu ] = useState<ContextMenuState>({open: false, x: 0, y: 0});
  const onPaneContextMenu = useCallback((e: React.MouseEvent) => { setMenu({open: true, x: e.clientX, y: e.clientY}); }, [setMenu]);
  const onMenuClose = useCallback(() => { setMenu({open: false, x: 0, y: 0}) }, [setMenu]);
  const onMenuClick = useCallback((node: Node) => { addNode(node); }, [setMenu]);

  return (
    <div className="container">
      <ReactFlow
        {...pipeline}
        onPaneContextMenu={onPaneContextMenu}
      >
        <MiniMap />
        <Controls />
        <Background />
      </ReactFlow>
      <Button onClick={onApply} primary>Apply</Button>
      <ContextMenu
        onMenuClose={onMenuClose}
        onMenuClick={onMenuClick}
        {...menu}
      />
    </div>
  );
}

export default App;
