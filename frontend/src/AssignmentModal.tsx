import { useCallback, useState } from 'react';
import 'reactflow/dist/style.css';

import { Button, Modal, Table, Select } from 'semantic-ui-react';

import { presentation } from '~/go/models';

type Prop = {
  open: boolean;
  edge?: presentation.Edge;
  source?: presentation.OutputPort;
  target?: presentation.InputPort;

  onAssign: (id: string, assignment: Record<string, string>) => void;
  onDiscard: () => void;
};

export const AssignmentModal: React.FC<Prop> = ({
  open,
  edge,
  source,
  target,
  onAssign,
  onDiscard,
}) => {
  const [assignment, setAssignment] = useState<Record<string, string>>(
    edge?.data.assignment ?? {},
  );
  const onClose = onDiscard;
  const onApply = useCallback(
    () => onAssign(edge?.id ?? '', assignment),
    [edge, onAssign, assignment],
  );

  return (
    <Modal open={open} onClose={onClose}>
      <Modal.Header>Edit Assignment</Modal.Header>
      <Modal.Content image scrolling>
        <Table>
          <Table.Header>
            <Table.Row>
              <Table.HeaderCell>Target</Table.HeaderCell>
              <Table.HeaderCell>Source</Table.HeaderCell>
            </Table.Row>
          </Table.Header>

          <Table.Body>
            {target?.propertyNames.map((prop: string) => (
              <Table.Row key={prop}>
                <Table.Cell>{prop}</Table.Cell>
                <Table.Cell>
                  <Select
                    value={edge?.data.assignment[prop]}
                    options={
                      source?.propertyNames.map((prop: string) => ({
                        key: prop,
                        value: prop,
                        text: prop,
                      })) ?? []
                    }
                    onChange={(_, { value }) => {
                      setAssignment({
                        ...assignment,
                        [prop]: value as string,
                      });
                    }}
                  />
                </Table.Cell>
              </Table.Row>
            ))}
          </Table.Body>
        </Table>
      </Modal.Content>
      <Modal.Actions>
        <Button onClick={onClose} secondary>
          Cancel
        </Button>
        <Button onClick={onApply} primary>
          Apply
        </Button>
      </Modal.Actions>
    </Modal>
  );
};
