import { DataProvider } from "react-admin";
import axios from 'axios';

const apiUrl = 'http://localhost:3000/api/graphql';

export const dataProvider: DataProvider = {
    // Fetch data for post listing
    getList: (resource, params) => {
        // Paginator status
        const { page, perPage } = params.pagination;
        // Sorter status
        const { field, order } = params.sort;

        // POST request to GraphQL endpoint
        return axios.post(apiUrl, {
            query: `
            query {
              notes (
                orderBy: { ${field}: ${order} },
                pagination: { page: { limit: ${perPage}, page: ${page - 1} }}
              ) {
                nodes {
                  id
                  title
                  createdAt
                  updatedAt
                }
                paginationInfo {
                  pages
                  current
                  offset
                  total
                }
              }
            }
            `
        })
            .then((response) => {
                // Unwrap the response
                const { nodes, paginationInfo } = response.data.data.notes;
                // Return the data array and total number of pages
                return {
                    data: nodes,
                    total: paginationInfo.total,
                };
            });
    },

    // Fetch data for a single post
    getOne: (resource, params) => {
        // POST request to GraphQL endpoint
        return axios.post(apiUrl, {
            query: `
            query {
              notes(filters: {id: {eq: ${params.id}}}) {
                nodes {
                  id
                  title
                  content
                  createdAt
                  updatedAt
                }
              }
            }
            `
        })
            .then((response) => {
                // Unwrap the response
                const { nodes } = response.data.data.notes;
                // Return the one and only data
                return {
                    data: nodes[0],
                };
            });
    },

    getMany: (resource, params) => { },

    getManyReference: (resource, params) => { },

    update: (resource, params) => { },

    updateMany: (resource, params) => { },

    create: (resource, params) => { },

    delete: (resource, params) => { },

    deleteMany: (resource, params) => { },
};
