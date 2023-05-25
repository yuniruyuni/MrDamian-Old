import { useCallback, useMemo, useEffect, useRef, DependencyList } from 'react';
import type { Edge as RFEdge, Connection, HandleType } from 'reactflow';
import { useNodesState, useEdgesState, updateEdge } from 'reactflow';

import { PropertiesNode } from './PropertiesNode';
import {
  editor,
  updateEditor,
  Node,
  Edge,
  InputPort,
  OutputPort,
} from './bindings';

import { listen, EventName, EventCallback } from '@tauri-apps/api/event';

function useListen<T>(
  event: EventName,
  handler: EventCallback<T>,
  deps?: DependencyList,
) {
  useEffect(() => {
    let unlisten: () => void = () => ({});
    (async () => {
      unlisten = await listen(event, handler);
    })();
    return () => {
      unlisten();
    };
  }, [event, handler, deps]);
}

export type Args = {
  onAssignEdit: (edge: Edge, source: OutputPort, target: InputPort) => void;
  onAddEdge: (connection: Connection) => void;
  onRemoveEdge: (connection: Connection) => void;
};

export function usePipeline({ onAssignEdit, onAddEdge, onRemoveEdge }: Args) {
  const edgeUpdateSuccessful = useRef(true);

  const nodeTypes = useMemo(
    () => ({
      TwitchSubscriber: PropertiesNode,
      TwitchPublisher: PropertiesNode,
    }),
    [],
  );
  const [nodes, setNodes, onNodesChange] = useNodesState([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);

  const rnodes: Node[] = useMemo(
    () => nodes.map((node) => ({ ...node, type: node.type ?? '' })),
    [nodes],
  );
  const redges: Edge[] = useMemo(
    () =>
      edges.map((edge) => ({
        ...edge,
        label: edge.label?.toString() ?? '',
        sourceHandle: edge.sourceHandle ?? '',
        targetHandle: edge.targetHandle ?? '',
        data: edge.data ?? {},
      })),
    [edges],
  );

  const onEdgeUpdateStart = useCallback(() => {
    edgeUpdateSuccessful.current = false;
  }, []);
  const onEdgeUpdate = useCallback(
    (oldEdge: RFEdge, newConnection: Connection) => {
      edgeUpdateSuccessful.current = true;
      setEdges((edges) => updateEdge(oldEdge, newConnection, edges));
    },
    [setEdges],
  );
  const onEdgeUpdateEnd = useCallback(
    (_event: MouseEvent | TouchEvent, target: RFEdge, _handle: HandleType) => {
      if (!edgeUpdateSuccessful.current) {
        setEdges((edges) => edges.filter((e) => e.id !== target.id));
        onRemoveEdge({
          source: target.source,
          target: target.target,
          sourceHandle: target.sourceHandle ?? '',
          targetHandle: target.targetHandle ?? '',
        });
      }
      edgeUpdateSuccessful.current = true;
    },
    [onRemoveEdge, setEdges],
  );

  const onConnect = useCallback(
    (connection: Connection) => onAddEdge(connection),
    [onAddEdge],
  );

  const onApply = useCallback(() => {
    (async () => {
      await updateEditor({ nodes: rnodes, edges: redges });
    })();
  }, [rnodes, redges]);

  const onEdgeClick = useCallback(
    (_e: React.MouseEvent, edge: RFEdge) => {
      const redge = redges.find((e: Edge) => e.id === edge.id);

      const source = rnodes.find((node: Node) => node.id === edge.source);
      const target = rnodes.find((node: Node) => node.id === edge.target);
      const sourcePort = source?.data.outputs.find(
        (i: OutputPort) => i.name === edge.sourceHandle,
      );
      const targetPort = target?.data.inputs.find(
        (o: InputPort) => o.name === edge.targetHandle,
      );

      if (!redge) return;
      if (!source) return;
      if (!target) return;
      if (!sourcePort) return;
      if (!targetPort) return;

      onAssignEdit(redge, sourcePort, targetPort);
    },
    [rnodes, redges, onAssignEdit],
  );

  useEffect(() => {
    (async () => {
      const { nodes, edges } = await editor();
      setNodes(nodes);
      setEdges(edges);
    })();
  }, [setNodes, setEdges]);

  useListen(
    'pipeline-updated',
    () => {
      (async () => {
        const { nodes, edges } = await editor();
        setNodes(nodes);
        setEdges(edges);
      })();
    },
    [setNodes, setEdges],
  );

  return {
    nodes,
    edges,
    nodeTypes,
    onNodesChange,
    onEdgesChange,
    onEdgeUpdateStart,
    onEdgeUpdate,
    onEdgeUpdateEnd,
    onConnect,
    onApply,
    onEdgeClick,
  };
}
